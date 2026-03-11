#include <stdio.h>
#include <stdlib.h>
#define ONE_DAY 86400

typedef struct{
	int start;
	int goal;
	int label;
}t_set;

int input_num(void){
	int n;
	scanf("%d",&n);
	return n;
}

t_set *input_time(int n){
	t_set *ret;
	int i;
	ret = (t_set *)malloc(sizeof(t_set)*n);
	
	for(i = 0;i < n;i++){
		int j, k, l;
		
		scanf("%d:%d:%d",&j,&k,&l);
		ret[i].start = j * 60 * 60+ k * 60 + l;
		
		scanf("%d:%d:%d",&j,&k,&l);
		ret[i].goal = j * 60 * 60 + k * 60 + l;
		
		ret[i].label = 0;
	}
	
	return ret;
}

int count_train(t_set *in, int n){
	int ret=0;
	int a;
	int label=0;
	int t;
	
	for(t = 0;t < ONE_DAY; t++){
		for(a = 0;a < n;a++){
			if(t == in[a].start && in[a].label == 0){
				in[a].label = 1;
				label ++;
				if(label > ret){
					ret = label;
				}
			}
			if(t == in[a].goal && label > 0){
				in[a].label = -1;
				label --;
			}
		}
	}
	return ret;
}

int main(){
	while(1){
		int t = 0;
		int n = input_num();
		int max = 0;
		int c = 0;
		
		t_set *time_set;
		if(n == 0){
			break;
		}
		
		time_set = (t_set *)malloc(sizeof(t_set)*n);
		time_set = input_time(n);
		
		c = count_train(time_set,n);
		
		printf("%d\n",c);
	}
	
	return 0;
}
//vim: set ft=C fenc=utf-8 ff=unix ts=4 sw=4: