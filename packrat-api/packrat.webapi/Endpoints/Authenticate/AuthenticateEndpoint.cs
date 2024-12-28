using FastEndpoints;
using Microsoft.AspNetCore.Http.HttpResults;
using packrat.Services.Services.Authentication;
using packrat.webapi.Common;
using packrat.webapi.Endpoints.Authenticate.Dtos;
using packrat.webapi.Endpoints.Registration.Dtos;

namespace packrat.webapi.Endpoints.Authenticate;

public class AuthenticateEndpoint : Endpoint<AuthenticateRequestDto, Results<Ok<AuthenticationResponseDto>, ProblemHttpResult, BadRequest>>
{
    private readonly IAuthenticationService _authenticationService;

    public AuthenticateEndpoint(IAuthenticationService authenticationService)
    {
        _authenticationService = authenticationService;
    }

    public override void Configure()
    {
        Post("authenticate");
        Version(1);
        AllowAnonymous();
    }

    public override async Task<Results<Ok<AuthenticationResponseDto>, ProblemHttpResult, BadRequest>> ExecuteAsync(AuthenticateRequestDto dto, CancellationToken ct)
    {
        try
        {
            var authenticated = await _authenticationService.Authenticate(dto.Email, dto.Password);
            if (!authenticated) return TypedResults.BadRequest();

            return TypedResults.Ok(new AuthenticationResponseDto());
        }
        catch (Exception ex)
        {
            Logger.LogError("Error authenticating user. Exception: {}", ex);
            return TypedResults.BadRequest();
        }
    }
}