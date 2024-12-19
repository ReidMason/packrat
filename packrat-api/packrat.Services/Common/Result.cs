namespace packrat.Services.Common;

public class Result<T, E>
{
    public T? Data { get; }
    public E? Errors { get; }

    public Result(T? data, E? errors)
    {
        Data = data;
        Errors = errors;
    }
}
