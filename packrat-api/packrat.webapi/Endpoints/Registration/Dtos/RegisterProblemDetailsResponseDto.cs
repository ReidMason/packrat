using packrat.Services.Services.Registration;

namespace packrat.webapi.Endpoints.Registration.Dtos;

public class RegisterProblemDetailsResponseDto
{
    public List<string> Email { get; } = [];
    public List<string> Password { get; } = [];

    public RegisterProblemDetailsResponseDto(RegisterValidationErrors validationErrors)
    {
        Email.AddRange(validationErrors.Email);
        Password.AddRange(validationErrors.Password);
    }
}
