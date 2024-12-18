using FastEndpoints;
using FastEndpoints.Swagger;
using packrat.databaseMigrations;
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

// Logging
Log.Logger = new LoggerConfiguration()
    .MinimumLevel.Information()
    .WriteTo.Console()
    .CreateLogger();
builder.Services.AddSerilog();

// Add services to the container.

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

var connectionString = app.Configuration.GetValue<string>("DatabaseConnection");
if (connectionString is null) {
    Log.Fatal("No connection string found");
    return;
}
DatabaseMigrator.MigrateDatabase(connectionString);

app.Run();