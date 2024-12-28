using System.Net.Mail;
using Microsoft.Extensions.Logging;
using packrat.dataAccessLayer.Services;
using packrat.Services.Common;
using packrat.Services.Services.Registration.Models;

namespace packrat.Services.Services.Registration;

public interface IRegistrationService
{
    public Task<Result<RegisteredUser, RegisterValidationErrors>> Register(string email, string password);
}

public class RegisterValidationErrors
{
    public List<string> Email { get; } = [];
    public List<string> Password { get; } = [];

    public bool IsError => Email.Count > 0 || Password.Count > 0;
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

       var emailValidation = ValidateEmail(email);
       if (!emailValidation.isValid) validationErrors.Email.Add("Invalid Email Address");
       email = emailValidation.email;

       var existingUser = await _userService.GetUserByEmail(email);
       if (existingUser is not null) validationErrors.Email.Add("Email already registered");

       if (password.Length < 6) validationErrors.Password.Add("Password must be at least 6 characters");

       var hashedPassword = BCrypt.Net.BCrypt.EnhancedHashPassword(password);

       if (validationErrors.IsError) return new Result<RegisteredUser, RegisterValidationErrors>(null, validationErrors);

       var newUser = await _userService.CreateUser(email, hashedPassword);
       return new Result<RegisteredUser, RegisterValidationErrors>(new RegisteredUser(newUser), null);
   }

   private (bool isValid, string email) ValidateEmail(string email)
   {
       try
       {
           email = new MailAddress(email).Address;
           return (true, email);
       }
       catch (FormatException)
       {
           return (false, email);
       }
   }
}