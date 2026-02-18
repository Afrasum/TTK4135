package elevator

type ElevatorId = int

const (
	Elevator1 ElevatorId = 1
	Elevator2 ElevatorId = 2
	Elevator3 ElevatorId = 3
)

type Floor = int

const (
	Floor0 Floor = 0
	Floor1 Floor = 1
	Floor2 Floor = 2
	Floor3 Floor = 3
)

type Elevator struct {
	ID          ElevatorId
	Floor       int
	Direction   bool
	IsIdle      bool
	CabOrders   [4]bool
	Obstruction bool
	StopButton  bool
	IsAlive     bool
	DoorOpen    bool
}

type HallOrders struct {
	Up   [4]bool
	Down [4]bool
}
