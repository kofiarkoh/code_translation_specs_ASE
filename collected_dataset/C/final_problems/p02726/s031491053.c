#include<stdio.h>

int sh(int i,int j,int x,int y){
	if(j<=x||y<=i){
		return j-i;
	}else if(x<=i&&j<=y){
		return (j-i)<(i-x+y-j+1)?(j-i):(i-x+y-j+1);
	}else if(j<=y){
		return (j-i)<(y-j+x-i+1)?(j-i):(y-j+x-i+1);
	}else if(x<=i){
		return (j-i)<(i-x+j-y+1)?(j-i):(i-x+j-y+1);
	}else{
		return j-i-(y-x)+1;
	}
}

int main(void){
	int n,x,y;
	scanf("%d %d %d",&n,&x,&y);
	int count[2000]={0};
	for(int i=1;i<n;i++){
		for(int j=i+1;j<=n;j++){
			count[sh(i,j,x,y)-1]+=1;
		}
	}
	for(int k=0;k<n-1;k++){
		printf("%d\n",count[k]);
	}
}
