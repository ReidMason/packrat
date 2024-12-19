using System.Net;
using FastEndpoints;
using packrat.Services.Services.Registration;

namespace packrat.webapi.Endpoints.Registration.Dtos;

public class RegisterRequestProblemDetailsResponseDto : IResult
{
    public List<string> Email { get; } = [];
    public List<string> Password { get; } = [];

    public RegisterRequestProblemDetailsResponseDto(RegisterValidationErrors validationErrors)
    {
        Email.AddRange(validationErrors.Email);
        Password.AddRange(validationErrors.Password);
    }

    public Task ExecuteAsync(HttpContext httpContext)
    {
        return httpContext.Response.SendAsync(this, (int)HttpStatusCode.BadRequest);
    }
}
