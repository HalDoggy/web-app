Push-Location
docker-compose -f $PSScriptRoot/../docker-compose.yml run --rm node sh -c "npm install -g create-react-app && create-react-app frontend-react"
Pop-Location