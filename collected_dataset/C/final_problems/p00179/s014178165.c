#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct NODE {
	char body[11];
	int  time;
	int  hash;
	struct NODE *next;
};

struct NODE *head;
struct NODE *tail;

int node_num = 0;

int change_worm(char *worm);	
void add_node(char *worm, int time);
int calc_hash(char *worm);
int change_color(struct NODE *node);
int check_body(char *worm);
int check_hash(int hash);
void free_node(void);

int main(void)
{
	char worm[11];
	int time;

	
	while (1){
		scanf("%s", worm);
		
		if (worm[0] == '0'){
			break;
		}
		time = change_worm(worm);
		if (time == -1){
			printf("NA\n");
		}
		else {
			printf("%d\n", time);
		}
		node_num = 0;
		
		free_node();
	}
	return (0);
}

int change_worm(char *worm)	
{
	struct NODE *node;
	int time;
	
	add_node(worm, 0);
	
	node = head;
	
	while (node != NULL){
//		printf("-->%d %s\n", node->time, node->body);
		time = change_color(node);
		if (time >= 0){
			return (time);
		}
		node = node->next;
	}
	return (-1);
}

void add_node(char *worm, int time)
{
	struct NODE *node;
	int hash;
	int i;
	
	node = malloc(sizeof(struct NODE));
	
	strcpy(node->body, worm);
	node->time = time;
	node->hash = calc_hash(node->body);
	node->next = NULL;
	
	if (head == NULL){
		head = tail = node;
	}
	else {
		tail->next = node;
		tail = node;
	}
//printf("<%d>\n", ++node_num);
//	printf("[%d  %s]\n", time, worm);
}
	
int calc_hash(char *worm)
{
	int hash;
	int i;
	
	hash = 0;
	i = 0;
	while (worm[i] != '\0'){
		hash *= 3;
		switch (worm[i]){
		  case 'r':
		    break;
			
		  case 'g':
		    hash++;
			break;
			
		  case 'b':
			hash += 2;
			break;
		}
		i++;
	}
	
	return (hash);
}

int change_color(struct NODE *node)
{
	int i;
	char new_worm[11];
	int hash;
	
	if (check_body(node->body) == 1){
		return (node->time);
	}

	i = 0;
	while (node->body[i + 1] != '\0'){
		if (node->body[i] != node->body[i + 1]){
			strcpy(new_worm, node->body);
			if (new_worm[i] != 'r' && new_worm[i + 1] != 'r'){
				new_worm[i] = new_worm[i + 1] = 'r';
			}
			else if (new_worm[i] != 'g' && new_worm[i + 1] != 'g'){
				new_worm[i] = new_worm[i + 1] = 'g';
			}
			else if (new_worm[i] != 'b' && new_worm[i + 1] != 'b'){
				new_worm[i] = new_worm[i + 1] = 'b';
			}
			
			if (check_body(new_worm) == 1){
				return (node->time + 1);
			}
			
			if (check_hash(calc_hash(new_worm)) == 1){
				add_node(new_worm, node->time + 1);
			}
		}
		i++;
	}
	
	return (-1);
}

int check_body(char *worm)
{
	int i;
	char first;
	
	first = worm[0];
	i = 1;
	while (worm[i] != '\0'){
		if (worm[i] != first){
			return (0);
		}
		i++;
	}
	
	return (1);
}

int check_hash(int hash)
{
	struct NODE *node;
	
	node = head;
	
	while (node != NULL){
		if (node->hash == hash){
//			printf("<<found>>\n");
			return (0);
		}
		node = node->next;
	}
	return (1);
}

void free_node(void)
{
	struct NODE *node;
	struct NODE *del_node;
	
	node = head;
	
	while (node->next != NULL){
		del_node = node;
		node = node->next;
		free(del_node);
	}
	head = tail = NULL;
}
		
		