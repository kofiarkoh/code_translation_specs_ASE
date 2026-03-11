#include<stdio.h>
int d[2010][2010],s[2010][2010];
int MAX(int a,int b){return a<b?b:a;}
int main(){
	int n,k,i,j,max=0,x,y;
	char c;
	scanf("%d %d",&n,&k);
	for(i=0;i<n;i++){
		scanf("%d %d %c",&x,&y,&c);//printf("%d\n",i);fflush(stdout);
		if((y%(2*k))/k)x+=k;
		if(c=='B')x+=k;
		x%=2*k;
		y%=k;//printf("%d %d\n",x,y);
		d[y][x]++;
		d[(y+k)%(2*k)][(x+k)%(2*k)]++;
	}//printf("a\n");
	for(i=0;i<k*2;i++){
		for(j=0;j<k*2;j++)s[i+1][j+1]=d[i][j]+s[i+1][j]+s[i][j+1]-s[i][j];
	}
	/*for(i=0;i<2*k;i++){
		for(j=0;j<2*k;j++)printf("%d ",d[i][j]);printf("\n");
	}printf("\n");
	for(i=0;i<2*k;i++){
		for(j=0;j<2*k;j++)printf("%d ",s[i][j]);printf("\n");
	}//*/
	for(i=0;i<k;i++){
		for(j=0;j<k;j++)max=MAX(max,s[i+k][j+k]+s[i][j]-s[i+k][j]-s[i][j+k]);
	}
	printf("%d\n",max);
	return 0;
}