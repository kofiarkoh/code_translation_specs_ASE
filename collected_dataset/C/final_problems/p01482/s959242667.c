#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>

#define OP_UNDEFINED	(-1000)
#define OP_NULL			(-2000)
#define OP_MALLOC		(-3000)
#define OP_CLONE		(-4000)
#define OP_FREE			(-5000)

int heap_size;
int heap_rest;

int memory_map_size;
int memory_map[10000];

char statement[400];
int operation_num;
int operations[1000];

int values[26];

int main(void) {
	int i;
	int current_value=0;
	int error=0;
	int leak_size;
	for(i=0;i<26;i++)values[i]=OP_UNDEFINED;
	scanf("%d",&heap_size);
	heap_rest=heap_size;
	while(scanf("%s",statement)==1) {
		operation_num=0;
		for(i=0;statement[i]!=0 && statement[i]!=')';) {
			if(statement[i]=='m') {
				operations[operation_num++]=OP_MALLOC;
				i+=7;
			} else if(statement[i]=='c') {
				operations[operation_num++]=OP_CLONE;
				i+=6;
			} else if(statement[i]=='f') {
				operations[operation_num++]=OP_FREE;
				i+=5;
			} else if(statement[i]=='N' && statement[i+1]=='U') {
				current_value=OP_NULL;
				break;
			} else if(isupper(statement[i])) {
				if(statement[i+1]=='=') {
					operations[operation_num++]=-(statement[i]-'A')-1;
					i+=2;
				} else {
					current_value=values[statement[i]-'A'];
					break;
				}
			} else if(statement[i]=='(') {
				i++;
			} else if(isdigit(statement[i])) {
				current_value=atoi(&statement[i]);
				break;
			} else return 1;
		}
		for(i=operation_num-1;i>=0;i--) {
			if(operations[i]==OP_MALLOC) {
				if(heap_rest<current_value) {
					current_value=OP_NULL;
				} else {
					memory_map[memory_map_size]=current_value;
					heap_rest-=current_value;
					current_value=memory_map_size;
					memory_map_size++;
				}
			} else if(operations[i]==OP_CLONE) {
				if(current_value!=OP_NULL) {
					if(current_value<0 ||
							current_value>=memory_map_size ||
							memory_map[current_value]<0) {
						error=1;
						current_value=OP_NULL;
					} else {
						if(heap_rest<memory_map[current_value]) {
							current_value=OP_NULL;
						} else {
							memory_map[memory_map_size]=memory_map[current_value];
							heap_rest-=memory_map[current_value];
							current_value=memory_map_size;
							memory_map_size++;
						}
					}
				}
			} else if(operations[i]==OP_FREE) {
				if(current_value!=OP_NULL) {
					if(current_value<0 ||
							current_value>=memory_map_size ||
							memory_map[current_value]<0) {
						error=1;
						current_value=OP_NULL;
					} else {
						heap_rest+=memory_map[current_value];
						memory_map[current_value]=OP_UNDEFINED;
					}
				}
			} else if(-26<=operations[i] && operations[i]<0) {
				values[-operations[i]-1]=current_value;
			} else return 1;
		}
	}
	if(error) {
		puts("Error");
	} else {
		for(i=0;i<26;i++) {
			if(0<=values[i] && values[i]<memory_map_size) {
				memory_map[values[i]]=OP_UNDEFINED;
			}
		}
		leak_size=0;
		for(i=0;i<memory_map_size;i++) {
			if(memory_map[i]!=OP_UNDEFINED)leak_size+=memory_map[i];
		}
		printf("%d\n",leak_size);
	}
	return 0;
}