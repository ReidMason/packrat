using System.Net;
using FastEndpoints;

namespace packrat.webapi.Common;

public class ProblemDetailsResponse<T>: IResult
{
    private readonly T _details;

    public ProblemDetailsResponse(T details)
    {
        _details = details;
    }

    public Task ExecuteAsync(HttpContext httpContext)
    {
        return httpContext.Response.SendAsync(_details, (int)HttpStatusCode.BadRequest);
    }
}