#!/bin/bash

SERVICE_NAME="hogger-bulk-worker"
SINGLE_WORKER_NAME="hogger-worker"

function start_hogger() {
    docker compose up --build -d
}

function list_containers() {
    echo "Running containers for $SERVICE_NAME and $SINGLE_WORKER_NAME:"
    docker ps --filter "ancestor=$(docker compose images -q $SERVICE_NAME)" --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"
    docker ps --filter "ancestor=$(docker compose images -q $SINGLE_WORKER_NAME)" --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"
    echo
}

function stop_container() {
    read -p "Enter container name to stop: " cname
    docker stop "$cname"
}

function stop_and_delete_container() {
    read -p "Enter container name to stop and delete: " cname
    docker rm -f "$cname"
}

function restart_container() {
    read -p "Enter container name to restart: " cname
    docker restart "$cname"
}

function restart_all() {
    docker ps --filter "ancestor=$(docker compose images -q $SERVICE_NAME)" --format "{{.Names}}" | while read cname; do
        docker restart "$cname"
    done
    docker ps --filter "ancestor=$(docker compose images -q $SINGLE_WORKER_NAME)" --format "{{.Names}}" | while read cname; do
        docker restart "$cname"
    done
}

function launch_instance() {
    read -p "Launch (b)ulk worker or (s)ingle worker? [b/s]: " wtype
    if [[ "$wtype" == "b" ]]; then
        read -p "Enter a unique name for the new bulk worker: " newname
        docker compose run -d --name "$newname" $SERVICE_NAME
    elif [[ "$wtype" == "s" ]]; then
        read -p "Enter a unique name for the new single worker: " newname
        docker compose run -d --name "$newname" $SINGLE_WORKER_NAME
    else
        echo "Invalid option."
    fi
}

function show_logs() {
    echo "Which service logs do you want to see?"
    echo "1) hogger"
    echo "2) mongo"
    echo "3) rabbitmq"
    echo "4) All bulk workers"
    echo "5) Specific bulk worker"
    echo "6) All single workers"
    echo "7) Specific single worker"
    echo "8) Back to menu"
    read -p "Choose an option: " logopt
    case $logopt in
        1) docker compose logs -f hogger ;;
        2) docker compose logs -f mongo ;;
        3) docker compose logs -f rabbitmq ;;
        4) 
            docker ps --filter "ancestor=$(docker compose images -q $SERVICE_NAME)" --format "{{.Names}}" | while read cname; do
                echo "===== Logs for $cname ====="
                docker logs "$cname"
                echo
            done
            ;;
        5)
            docker ps --filter "ancestor=$(docker compose images -q $SERVICE_NAME)" --format "{{.Names}}"
            read -p "Enter bulk worker container name: " cname
            docker logs -f "$cname"
            ;;
        6)
            docker ps --filter "ancestor=$(docker compose images -q $SINGLE_WORKER_NAME)" --format "{{.Names}}" | while read cname; do
                echo "===== Logs for $cname ====="
                docker logs "$cname"
                echo
            done
            ;;
        7)
            docker ps --filter "ancestor=$(docker compose images -q $SINGLE_WORKER_NAME)" --format "{{.Names}}"
            read -p "Enter single worker container name: " cname
            docker logs -f "$cname"
            ;;
        8) return ;;
        *) echo "Invalid option";;
    esac
}

function main_menu() {
    while true; do
        echo "Bulk/Single Worker Management Menu"
        echo "---------------------------------"
        echo "1) Start Hogger (docker compose up --build -d)"
        echo "2) List running worker containers"
        echo "3) Stop a container"
        echo "4) Stop and delete a container"
        echo "5) Restart a container"
        echo "6) Restart all worker containers"
        echo "7) Launch a new worker instance"
        echo "8) See logs"
        echo "9) Exit"
        echo
        read -p "Choose an option: " opt
        case $opt in
            1) start_hogger ;;
            2) list_containers ;;
            3) list_containers; stop_container ;;
            4) list_containers; stop_and_delete_container ;;
            5) list_containers; restart_container ;;
            6) restart_all ;;
            7) launch_instance ;;
            8) show_logs ;;
            9) exit 0 ;;
            *) echo "Invalid option";;
        esac
        echo
    done
}

main_menu