#include<stdio.h>
int MAX(int a,int b){return a<b?b:a;}
int n,m,i,j,max=0,s[2010][3010],d[2010][3010];
int main(){
	char c;
	scanf("%d %d",&n,&m);
	while(n--){
		scanf("%d %d %c",&j,&i,&c);
		if(c=='B')j+=m;
		if(i%(2*m)/m)j+=m;
		d[i%m][j%(m*2)]++;
	}
	for(i=0;i<m;i++){
		for(j=0;j<2*m;j++)d[i+m][(j+m)%(2*m)]=d[j][i+2*m]=d[i][j];
	}
	for(i=0;i<2*m;i++){
		for(j=0;j<3*m;j++)s[i+1][j+1]=d[i][j]+s[i+1][j]+s[i][j+1]-s[i][j];
	}
	for(i=0;i<=m;i++){
		for(j=0;j<2*m;j++)max=MAX(max,s[i][j]+s[i+m][j+m]-s[i+m][j]-s[i][j+m]);
	}
	printf("%d\n",max);
	return 0;
}