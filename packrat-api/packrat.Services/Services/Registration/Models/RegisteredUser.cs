using packrat.dataAccessLayer.Context;

namespace packrat.Services.Services.Registration.Models;

public class RegisteredUser
{
    public long Id { get; set; }
    public string Email { get; set; }

    public RegisteredUser(User user)
    {
       Id = user.Id;
       Email = user.Email;
    }
}