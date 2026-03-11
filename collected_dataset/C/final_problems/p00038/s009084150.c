#include <stdio.h>
#include <math.h>
#include <string.h>
#include <ctype.h>

int main(){
	int num[5],i,j,k,l,m,key,flag=0,count=0;
	
	while(scanf("%d,%d,%d,%d,%d",&num[0],&num[1],&num[2],&num[3],&num[4])!=EOF){
	
	for(i=0;i<5;i++){
		for(j=i;j<5;j++)if(num[i]==num[j] && i!=j){
			flag=1;
			count++;
		}
	}
	
	if(count==2)flag=2;
	
	for(i=0;i<5;i++){
		for(j=i;j<5;j++){
			if(num[i]==num[j] && i!=j){
				for(k=0;k<5;k++)if(i!=k && j!=k && num[i]==num[k] && num[j]==num[k])flag=3;
			}
		}
	}
	
	for(i=0;i<5;i++){
		for(j=i;j<5;j++){
			if(num[i]>num[j]){
				key=num[i];
				num[i]=num[j];
				num[j]=key;
			}
		}
	}
	if(num[0]==num[1]-1 && num[1]==num[2]-1 && num[2]==num[3]-1 && num[3]==num[4]-1 || num[0]==1 && num[1]==10 && num[2]==11 && num[3]==12 && num[4]==13)flag=4;
	
	for(i=0;i<5;i++){
		for(j=i;j<5;j++){
			if(num[i]==num[j] && i!=j){
				for(k=0;k<5;k++){
					if(i!=k && j!=k && num[i]==num[k] && num[j]==num[k]){
						for(l=0;l<5;l++){
							for(m=l;m<5;m++)if(num[l]==num[l+m] && m!=i && m!=j && m!=k && m!=l)flag=5;
						}
					}
				}
			}
		}
	}
	
	for(i=0;i<5;i++){
		for(j=i;j<5;j++){
			if(num[i]==num[j] && i!=j){
				for(k=0;k<5;k++){
					if(i!=k && j!=k && num[i]==num[k] && num[j]==num[k]){
						for(l=0;l<5;l++)if(i!=l && j!=l && k!=l && num[i]==num[l] && num[j]==num[l] && num[k]==num[l] )flag=6;
					}
				}
			}
		}
	}
	
	switch(flag){
	case 0:
		printf("null\n");
		break;
	case 1:
		printf("one pair\n");
		break;
	case 2:
		printf("two pair\n");
		break;
	case 3:
		printf("three card\n");
		break;
	case 4:
		printf("straight\n");
		break;
	case 5:
		printf("full house\n");
		break;
	case 6:
		printf("four card\n");
		break;
	}
		flag=0;
		count=0;
	}
return 0;
}