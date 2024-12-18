using System;
using System.Collections.Generic;

namespace packrat.dataAccessLayer.Context;

public partial class User
{
    public long Id { get; set; }

    public string Email { get; set; } = null!;

    public DateTime DateCreated { get; set; }

    public DateTime DateModified { get; set; }
}
