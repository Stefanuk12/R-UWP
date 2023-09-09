-- // Dependencies
local SignalManager = loadstring(game:HttpGet("https://raw.githubusercontent.com/Stefanuk12/Signal/main/Manager.lua"))().new()
SignalManager:Add("ServerResponse")

-- // Automatically manages connecting to websockets
local WebsocketManager = {}
WebsocketManager.__index = WebsocketManager
do
    -- // Constructor
    function WebsocketManager.new(URL, ReconnectTime)
        -- // Create the object
        local self = setmetatable({}, WebsocketManager)

        -- // Vars
        self.Connection = nil
        self.ReconnectTime = ReconnectTime or 5
        self.PendingJobs = {}

        -- // Connect
        self:Connect(URL)

        -- // Return the object
        return self
    end

    -- // Decodes the response
    function WebsocketManager:Decode(Message)
        -- // Extract the Job Id
        local Data = Message:split("|")
        local JobId = Data[1]
        table.remove(Data, 1)

        -- // Return the Job Id and the data
        return JobId, Data
    end

    -- // Connects to the websocket server
    function WebsocketManager:Connect(url)
        -- // Make sure we are not already connected
        assert(not self.Connection, "already connected to websocket server")

        -- // Connect
        self.Connection = WebSocket.connect(url)

        -- // Listen to when the socket closes
        self.Connection.OnClose:Connect(function()
            -- // Stop
            self.Connection = nil

            -- // Attempt to reconnect
            while true do task.wait(self.ReconnectTime)
                -- // Attempt to connect
                local Success, _ = pcall(self.Connect, self, url)

                -- // Check if we were successful
                if (Success) then
                    break
                end
            end
        end)

        -- // Listen to when the socket receives a message
        self.Connection.OnMessage:Connect(function(message)
            -- // Decode the message
            local JobId, Data = self:Decode(message)

            -- // Remove the job from the pending jobs
            local i = table.find(self.PendingJobs, JobId)
            if (i) then
                table.remove(self.PendingJobs, i)
            end

            -- // Fire
            SignalManager:Fire("ServerResponse", JobId, Data)
        end)
    end

    -- // Wait for a response from the server with a certain Job Id
    function WebsocketManager:WaitForResponse(JobId, Timeout)
        -- // Wait for the response
        local RJobId, RData = SignalManager:Wait("ServerResponse", Timeout, function(ReceivedJobId, _Data)
            return ReceivedJobId == JobId
        end)

        -- // Return the response (JobId, Data)
        return RJobId, RData
    end

    -- // Send a request to the websocket server
    function WebsocketManager:Send(CommandId, ...)
        -- // Make sure we are connected
        assert(self.Connection, "not connected to websocket server")

        -- // Generate a job id
        local JobId = tostring(#self.PendingJobs)

        -- // Parse the message
        local Message = JobId .. "|" .. CommandId .. "|" .. table.concat({...}, "|")

        -- // Send the message and store the Job Id
        table.insert(self.PendingJobs, JobId)
        self.Connection:Send(Message)

        -- // Return the Job Id
        return JobId
    end

    -- // Send a request to the websocket server and wait for a response
    function WebsocketManager:SendAwait(Timeout, CommandId, ...)
        -- // Send the request
        local JobId = self:Send(CommandId, ...)

        -- // Wait for the response
        return self:WaitForResponse(JobId, Timeout)
    end
end

-- // Attempt to connect
local Connection = WebsocketManager.new("ws://localhost:8080")

getgenv().mousemoverel = function(x, y)
    -- // Assert x, y are numbers
    assert(type(x) == "number", "x must be a number")
    assert(type(y) == "number", "y must be a number")

    -- // Send the request
    Connection:Send(
        "0",
        x,
        y
    )
end

getgenv().mousemoveabs = function(x, y)
    -- // Assert x, y are numbers
    assert(type(x) == "number", "x must be a number")
    assert(type(y) == "number", "y must be a number")

    -- // Send the request
    Connection:Send(
        "1",
        x,
        y
    )
end

getgenv().setclipboard = function(x)
    -- // Assert x is a string
    assert(type(x) == "string", "x must be a string")

    -- // Send the request
    Connection:Send(
        "2",
        x
    )
end

getgenv().mouse1click = function()
    -- // Send the request
    Connection:Send(
        "3"
    )
end