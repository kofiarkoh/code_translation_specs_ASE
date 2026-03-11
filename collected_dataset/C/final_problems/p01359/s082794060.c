#include <stdio.h>

typedef struct {
	char name[20];
	int start,max;
} era_t;

int era_max,query_max;
era_t era[1000];

int main(void) {
	int i,j;
	int query;
	int found;
	while(1) {
		scanf("%d%d",&era_max,&query_max);
		if(era_max==0 && query_max==0)break;
		for(i=0;i<era_max;i++) {
			scanf("%s%d%d",&era[i].name,&era[i].max,&era[i].start);
			era[i].start-=era[i].max-1;
		}
		for(i=0;i<query_max;i++) {
			scanf("%d",&query);
			found=0;
			for(j=0;j<query_max;j++) {
				if(era[j].start<=query && query-era[j].start+1<=era[j].max) {
					printf("%s %d\n",era[j].name,query-era[j].start+1);
					found=1;
					break;
				}
			}
			if(!found)puts("Unknown");
		}
	}
	return 0;
}