using FastEndpoints;

namespace packrat.webapi.Endpoints.Health;

public class GetHealthEndpoint : EndpointWithoutRequest<HealthResponseDto>
{
    public override void Configure()
    {
        Get("/health");
        AllowAnonymous();
    }

    public override async Task HandleAsync(CancellationToken ct)
    {
        await SendAsync(new HealthResponseDto("Healthy"), cancellation: ct);
    }
}