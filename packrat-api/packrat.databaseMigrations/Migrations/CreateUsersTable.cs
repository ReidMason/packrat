using FluentMigrator;

namespace packrat.databaseMigrations.Migrations;

[Migration(1)]
public class CreateUsersTable : Migration
{
    public override void Up()
    {
        Create.Table("users")
            .WithColumn("id").AsInt64().PrimaryKey().Identity()
            .WithColumn("username").AsString();
    }

    public override void Down()
    {
        Delete.Table("users");
    }
}