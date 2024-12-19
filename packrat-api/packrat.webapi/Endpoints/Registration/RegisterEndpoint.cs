using FastEndpoints;
using Microsoft.AspNetCore.Http.HttpResults;
using packrat.Services.Services.Registration;
using packrat.Services.Services.Registration.Models;
using packrat.webapi.Common;
using packrat.webapi.Endpoints.Registration.Dtos;

namespace packrat.webapi.Endpoints.Registration;

public class RegisterEndpoint : Endpoint<RegisterRequestDto, Results<Ok<RegisteredUser>, ProblemDetailsResponse<RegisterProblemDetailsResponseDto>, ProblemHttpResult>>
{
    private readonly IRegistrationService _registrationService;

    public RegisterEndpoint(IRegistrationService registrationService)
    {
        _registrationService = registrationService;
    }

    public override void Configure()
    {
        Post("register");
        Version(1);
        AllowAnonymous();
    }

    public override async Task<Results<Ok<RegisteredUser>, ProblemDetailsResponse<RegisterProblemDetailsResponseDto>, ProblemHttpResult>> ExecuteAsync(RegisterRequestDto dto, CancellationToken ct)
    {
        try
        {
            var newUserResult = await _registrationService.Register(dto.Email, dto.Password);
            if (newUserResult.Errors is not null)
                return new ProblemDetailsResponse<RegisterProblemDetailsResponseDto>(new RegisterProblemDetailsResponseDto(newUserResult.Errors));

            return TypedResults.Ok(newUserResult.Data);
        }
        catch (Exception ex)
        {
            Logger.LogError("Error registering user. Exception: {}", ex);
            return TypedResults.Problem();
        }
    }
}