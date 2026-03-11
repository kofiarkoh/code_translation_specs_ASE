#include <stdio.h>

char input[100004];

int isok(char prev,char now,int nowisleft) {
	if(prev==now)return 0;
	if(nowisleft) {
		if(prev=='U' || prev=='D') {
			if(now=='R')return 0;
		} else if(prev=='L')return 0;
	} else {
		if(prev=='U' || prev=='D') {
			if(now=='L')return 0;
		} else if(prev=='R')return 0;
	}
	return 1;
}

int main(void) {
	int data_num,cur_data;
	int i;
	int isyes;
	scanf("%d",&data_num);
	for(cur_data=0;cur_data<data_num;cur_data++) {
		scanf("%s",input);
		isyes=1;
		for(i=1;input[i];i++) {
			if(!isok(input[i-1],input[i],i%2)) {
				isyes=0;
				break;
			}
		}
		if(!isyes) {
			isyes=1;
			for(i=1;input[i];i++) {
				if(!isok(input[i-1],input[i],!(i%2))) {
					isyes=0;
					break;
				}
			}
		}
		puts(isyes?"Yes":"No");
	}
	return 0;
}