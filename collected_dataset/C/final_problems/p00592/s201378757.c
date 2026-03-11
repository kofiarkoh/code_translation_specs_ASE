#include <stdio.h>
#include <stdlib.h>

int main(void) {
	int channel_num,start_time,end_time;
	int* cm_channels;
	int cm_num;
	int cm_start,cm_end;
	int i,j;
	int now_result,max_result;
	while(1) {
		scanf("%d%d%d",&channel_num,&start_time,&end_time);
		if(channel_num==0 && start_time==0 && end_time==0)break;
		end_time-=start_time;
		if(end_time<0){puts("0");continue;}
		cm_channels=calloc(end_time+1,sizeof(int));
		if(cm_channels==NULL)return 1;//error
		for(i=0;i<channel_num;i++) {
			scanf("%d",&cm_num);
			for(j=0;j<cm_num;j++) {
				scanf("%d%d",&cm_start,&cm_end);
				cm_start-=start_time;
				cm_end-=start_time;
				if(cm_start<0)cm_start=0;
				if(cm_start>end_time)cm_start=end_time;
				if(cm_end<0)cm_end=0;
				if(cm_end>end_time)cm_end=end_time;
				cm_channels[cm_start]++;
				cm_channels[cm_end]--;
			}
		}
		for(i=1;i<=end_time;i++)cm_channels[i]+=cm_channels[i-1];
		max_result=0;
		now_result=0;
		for(i=0;i<end_time;i++) {
			if((i+start_time)%100>=60) {
				i+=100-(i+start_time)%100;
			}
			if(i>=end_time)break;
			if(cm_channels[i]<channel_num)now_result++;
			else {
				if(max_result<now_result)max_result=now_result;
				now_result=0;
			}
		}
		if(max_result<now_result)max_result=now_result;
		printf("%d\n",max_result);
		free(cm_channels);
	}
	return 0;
}