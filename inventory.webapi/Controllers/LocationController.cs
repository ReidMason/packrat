using Microsoft.AspNetCore.Mvc;
using inventory.client.dtos;
using inventory.service.services;

namespace inventory.webapi.Controllers;

[ApiController]
[Route("[controller]")]
public class LocationsController : ControllerBase
{
    private readonly ILogger<LocationsController> _logger;
    private readonly IInventoryService _inventoryService;

    public LocationsController(ILogger<LocationsController> logger, IInventoryService inventoryService)
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
      _inventoryService.CreateLocation(newLocation);
      return Ok();
    }
}

