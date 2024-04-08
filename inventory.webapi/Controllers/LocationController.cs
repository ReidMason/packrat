using inventory.client.dtos;
using inventory.service.services;
using Microsoft.AspNetCore.Mvc;

namespace inventory.webapi.Controllers;

[ApiController]
[Route("[controller]")]
public class LocationsController : ControllerBase
{
    private readonly ILogger<LocationsController> _logger;
    private readonly IInventoryService _inventoryService;

    public LocationsController(
        ILogger<LocationsController> logger,
        IInventoryService inventoryService
    )
    {
        _logger = logger;
        _inventoryService = inventoryService;
    }

    [HttpGet]
    public IActionResult GetAllLocations()
    {
        return Ok(_inventoryService.GetAllLocations());
    }

    [HttpPost]
    public IActionResult CreateLocation(NewLocationDto newLocation)
    {
        var result = _inventoryService.CreateLocation(newLocation);

        if (result.Error == Errors.LocationAlreadyExists)
            return Conflict("Location already exists");

        if (!result.IsSuccessful)
        {
            _logger.LogError("Error creating location");
            return StatusCode(500, "Error creating location");
        }

        return Created();
    }
}
