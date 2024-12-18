using FluentMigrator.Runner;
using Microsoft.Extensions.DependencyInjection;
using packrat.databaseMigrations.Migrations;

namespace packrat.databaseMigrations;

public abstract class DatabaseMigrator
{
    public static void MigrateDatabase(string connString)
    {
        using (var serviceProvider = CreateServices(connString))
        using (var scope = serviceProvider.CreateScope())
        {
            UpdateDatabase(scope.ServiceProvider);
        }
    }

    private static ServiceProvider CreateServices(string connString)
    {
        return new ServiceCollection()
            .AddFluentMigratorCore()
            .ConfigureRunner(rb => rb
                .AddPostgres()
                .WithGlobalConnectionString(connString)
                .ScanIn(typeof(CreateUsersTable).Assembly).For.Migrations())
            .AddLogging(lb => lb.AddFluentMigratorConsole())
            .BuildServiceProvider(false);
    }

    private static void UpdateDatabase(IServiceProvider serviceProvider)
    {
        var runner = serviceProvider.GetRequiredService<IMigrationRunner>();

        runner.MigrateUp();
    }
}