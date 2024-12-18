using FastEndpoints;
using Microsoft.AspNetCore.Http.HttpResults;
using packrat.Services.Services.Registration;
using packrat.Services.Services.Registration.Models;
using packrat.webapi.Endpoints.Authentication.Dtos;

namespace packrat.webapi.Endpoints.Authentication;

public class RegisterEndpoint : Endpoint<RegisterRequestDto, Results<Ok<RegisteredUser>, ProblemDetails, ProblemHttpResult>>
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

    public override async Task<Results<Ok<RegisteredUser>, ProblemDetails, ProblemHttpResult>> ExecuteAsync(RegisterRequestDto dto, CancellationToken ct)
    {
        try
        {
            var newUserResult = await _registrationService.Register(dto.Email, dto.Password);
            if (newUserResult.Error is not null)
            {
                foreach (var validationFailure in newUserResult.Error.Email)
                    AddError(r => r.Email, validationFailure);

                return new ProblemDetails(ValidationFailures);
            }

            return TypedResults.Ok(newUserResult.Data);
        }
        catch (Exception ex)
        {
            Logger.LogError("Error registering user. Exception: {}", ex);
            return TypedResults.Problem();
        }
    }
}