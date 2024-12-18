using Microsoft.Extensions.Logging;
using packrat.dataAccessLayer.Services;
using packrat.Services.Services.Registration.Models;

namespace packrat.Services.Services.Registration;

public interface IRegistrationService
{
    public Task<Result<RegisteredUser, RegisterValidationErrors>> Register(string email, string password);
}

public class RegisterValidationErrors
{
    public List<string> Email { get; set; } = [];
    public List<string> Password { get; set; } = [];
}

public class Result<T, E>
{
    public T? Data { get; }
    public E? Error { get; }

    public Result(T? data, E? error)
    {
        Data = data;
        Error = error;
    }
}

public class RegistrationService : IRegistrationService
{
   private readonly ILogger<RegistrationService> _logger;
   private readonly IUserDbService _userService;

   public RegistrationService(ILogger<RegistrationService> logger, IUserDbService userService)
   {
       _logger = logger;
       _userService = userService;
   }

   public async Task<Result<RegisteredUser, RegisterValidationErrors>> Register(string email, string password)
   {
       var validationErrors = new RegisterValidationErrors();

       var existingUser = await _userService.GetUserByEmail(email);
       if (existingUser is not null)
       {
           validationErrors.Email.Add($"Email {email} is already registered");
           return new Result<RegisteredUser, RegisterValidationErrors>(null, validationErrors);
       }

       var newUser = await _userService.CreateUser(email, password);
       return new Result<RegisteredUser, RegisterValidationErrors>(new RegisteredUser(newUser), null);
   }
}