#include<stdio.h>
#include<string.h>
int main(){
	int x,y,m[20][20],d[20][20],i,j,ans;
	FILE *fp;
	//fp=fopen("test.txt","r");
	fp=stdin;
	while(1){
		fscanf(fp,"%d%d",&x,&y);
		if(x==0 && y==0)break;
		memset(d,0,sizeof(d));
		memset(m,0,sizeof(m));
		for(i=1;i<=y;i++){
			for(j=1;j<=x;j++){
				fscanf(fp,"%d",&m[i][j]);
				if(i==1 && m[i][j]!=1)d[i][j]=1;
			}
		}
		for(i=1;i<=y;i++){
			for(j=1;j<=x;j++){
				if(m[i][j]==0){
					d[i][j]+=d[i-1][j-1]+d[i-1][j]+d[i-1][j+1];
				}else if(m[i][j]==1){
					d[i][j]=0;
				}else if(m[i][j]==2){
					d[i+2][j]+=d[i-1][j]+d[i][j];
					d[i][j]=0;
				}
				//printf("%d%s",d[i][j],(j==x)?"\n":" ");
			}
		}
		for(i=1;i<=x;i++){
			d[y+1][i]+=d[y][i];
			//printf("%d%s",d[y+1][i],(i==x)?"\n":" ");
		}
		ans=0;
		for(i=1;i<=x;i++)ans+=d[y+1][i]/*+d[y][i]*/;
		printf("%d\n",ans);
	}
	return 0;
}