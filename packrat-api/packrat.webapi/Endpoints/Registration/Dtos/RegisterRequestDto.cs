namespace packrat.webapi.Endpoints.Registration.Dtos;

public class RegisterRequestDto
{
    public required string Email { get; set; }
    public required string Password { get; set; }
}