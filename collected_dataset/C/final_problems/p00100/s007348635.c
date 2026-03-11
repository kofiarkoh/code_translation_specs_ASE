#include<stdio.h>

int main(){
	int kazu;
	long long data[4000][4];
	int kaunto[4000][2];
	int count;
	int i,j;
	int number=0;
	
	for(i=0;i<4000;i++){
		data[i][0]=i+1;
		kaunto[i][0]=i+1;
	}
	for(i=0;i<4000;i++){
		for(j=1;j<4;j++){
			data[i][j]=0;
		}
	}
	for(i=0;i<4000;i++){
		kaunto[i][1]=0;
	}
	
	scanf("%d",&kazu);
	while(kazu!=0){
		count=0;
		
		for(i=0;i<kazu;i++){
			scanf("%d",&number);
			scanf("%lld %lld",&data[number-1][1],&data[number-1][2]);
			data[number-1][3]+=data[number-1][1]*data[number-1][2];
		}
				
		for(i=0;i<4000;i++){
			if(data[i][3]>=1000000 && kaunto[i][1]==0){
				printf("%d\n",kaunto[i][0]);
				kaunto[i][1]=1;
				count=1;
			}
		}
		if(count==0){
			printf("NA\n");
		}
		
		scanf("%d",&kazu);
	}
	return 0;
}