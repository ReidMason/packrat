using FastEndpoints;
using FastEndpoints.Swagger;
using Microsoft.EntityFrameworkCore;
using packrat.dataAccessLayer.Context;
using packrat.dataAccessLayer.Services;
using packrat.databaseMigrations;
using packrat.Services.Services.Registration;
using Serilog;

var builder = WebApplication.CreateBuilder(args);
builder.Services
    .AddFastEndpoints()
    .SwaggerDocument(o =>
    {
        o.MaxEndpointVersion = 1;
        o.DocumentSettings = s =>
        {
            s.DocumentName = "v1";
            s.Version = "v1";
        };
    });
builder.Services.AddSwaggerGen();

// Logging
Log.Logger = new LoggerConfiguration()
    .MinimumLevel.Information()
    .WriteTo.Console()
    .CreateLogger();
builder.Services.AddSerilog();

// Connection strings
var connectionString = builder.Configuration.GetValue<string>("DatabaseConnection");

// Add services to the container.
builder.Services.AddDbContext<PackratContext>(options => options.UseNpgsql(connectionString));
builder.Services.AddTransient<IUserDbService, UserDbService>();
builder.Services.AddTransient<IRegistrationService, RegistrationService>();

var app = builder.Build();
app.UseFastEndpoints(c =>
{
    c.Versioning.Prefix = "v";
    c.Versioning.PrependToRoute = true;
    c.Endpoints.RoutePrefix = "api";
})
.UseSwaggerGen();

// Configure the HTTP request pipeline.
app.UseHttpsRedirection();

if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

if (connectionString is null) {
    Log.Fatal("No connection string found");
    return;
}
DatabaseMigrator.MigrateDatabase(connectionString);

app.Run();