#include <stdlib.h>
#include <stdio.h>
#include <sys/types.h>
#include <errno.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>

#define PIPE "/tmp/nervous_system"

int vision(char *buf){
	char* DATA[1000];
	char ID;
	char ACTION[20];
	char INFO[100];
	char INFOS[5][30];
	int i = 0, j = 0;

	char *token = strtok(buf, "/");
	while (token != NULL){
		if (i==0){
			strcpy(&ID, token);
		} else if (i==1) {
			strcpy(ACTION, token);
		} else if (i==2) {
			strcpy(INFO, token);
			char *INFO_TOK = strtok(INFO, ",");
			while (INFO_TOK != NULL && j<5){
				strcpy(INFOS[j],INFO_TOK);
				INFO_TOK = strtok(NULL, ",");
				j++;
			}	
		} else { printf("String not correctly formated"); }
		token = strtok(NULL, "/");
		i++;
	}
	return 0;
}
int nervous(){ // Handles the IO of pipe as a stream
	int fd;
	char buffer[256];
	fd = open(PIPE,O_RDONLY);
	if (fd == -1){
		perror("Can't handle the nervous system... broken pipe ?");
		return 1;
	}
	while(1){
		ssize_t bytesRead = read(fd, buffer, sizeof(buffer) - 1);
		if (bytesRead == -1){
			perror("Failed to read from FIFO :(");
			close(fd);
			return 1;
		} else if (strstr(buffer,"1") != NULL) {
			vision("hello");	
		} else {
			perror("Message is wrong... corruption ? mod ?"); 
		}
	}
}

void main(){
	pid_t pid = fork();
	char a[56];
	if(pid<0){
		perror("Fork fail");
		exit(1);
	} else if (pid == 0) {
		// Child process monitors the pipe
		scanf("%s", &a);
		nervous();
	} else {
		// Parent process, rest of the program here
	}
}
