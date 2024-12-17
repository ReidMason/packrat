namespace packrat.webapi.Endpoints.Health;

public class HealthResponseDto
{
    public string Status { get; set; }

    public HealthResponseDto(string status)
    {
        Status = status;
    }
}