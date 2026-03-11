#include<stdio.h>

int same(long int goal[], long int now[], int n){
	int same=0;
	for(int i=1; i<=n; i++){
		if(goal[i] == now[i]) same++;
	}
	return (same);
}

int small(long int goal[], long int now[], int n){
	for(int i=1; i<=n; i++){
		if((now[i]-goal[i]) < 0){
			return -1;
		}
	}
	return 0;
}

int main(){
	int n;
	scanf("%d", &n);
	long int goal[n+1], now[n+1];
	goal[0] = -100, now[0] = -100;
	for(int i=1; i<=n; i++){
		scanf("%ld", &goal[i]);
	}
	for(int i=1; i<=n; i++){
		scanf("%ld", &now[i]);
	}
	//入力
	int count=0, max=0, max_n=0;
	
	for(int i=1; i<=1000000000; i++){
		if(same(goal, now, n) == n){
			printf("%d", count);
			break;
		}
		else if(count == 0){
			if(small(goal, now, n) == -1){
				printf("%d", -1);
				break;
			}
		}
		
		max=0, max_n=0;
		for(int i=1; i<=n; i++){
			if(max < now[i]){
				max=now[i];
				max_n=i;
			}
		}
		if(max_n == 1){
			now[1] = now[1]-now[2]-now[n];
			if((now[1]-goal[1]) < 0){
				printf("%d", -1);
					break;
			}
		}
		else if(max_n == n){
			now[n] = now[n]-now[n-1]-now[1];
			if((now[n]-goal[n]) < 0){
				printf("%d", -1);
					break;
			}
		}
		else{
			now[max_n] = now[max_n]-now[max_n+1]-now[max_n-1];
			if((now[max_n]-goal[max_n]) < 0){
				printf("%d", -1);
					break;
			}
		}
		count++;
	}
	
	return 0;
}