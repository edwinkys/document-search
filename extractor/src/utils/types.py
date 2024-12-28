class HeartbeatResponse:
    version: str

    def __init__(self, version: str):
        self.version = version

    def __str__(self):
        return f"version: {self.version}"
