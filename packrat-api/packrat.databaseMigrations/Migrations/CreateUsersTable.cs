using FluentMigrator;

namespace packrat.databaseMigrations.Migrations;

[Migration(1)]
public class CreateUsersTable : Migration
{
    public override void Up()
    {
        Create.Table("users")
            .WithColumn("id").AsInt64().PrimaryKey().Identity()
            .WithColumn("email").AsString().Unique().NotNullable()
            .WithColumn("password").AsString().NotNullable()
            .WithColumn("date_created").AsDateTime().NotNullable().WithDefault(SystemMethods.CurrentDateTime)
            .WithColumn("date_modified").AsDateTime().NotNullable().WithDefault(SystemMethods.CurrentDateTime);
    }

    public override void Down()
    {
        Delete.Table("users");
    }
}