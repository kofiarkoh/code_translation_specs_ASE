#include<stdio.h>
#include<string.h>
int main(){
	int d[105][105];
	int i,j,k,f,n,s,x,y,gx,gy;
	scanf("%d",&n);
	for(k=0;k<n;k++){
		memset(d,0,sizeof(d));
		scanf("%d",&s);
		x=0;y=s-1;f=0;
		if(s%2==1){
			if(((s+1)/2)%2==0)gx=gy=(s+1)/2;
			else gx=gy=(s-1)/2;
		}else{
			if((s/2)%2==0)gx=gy=s/2,gy++;
			else gx=gy=s/2,gy--;
		}
		while(1){
			if(x==gx && y==gy)break;
			d[y][x]=1;
			switch(f){
				case 0:
					if(y==0 ||(y>=2 && d[y-2][x]==1))f=1;
					else y--;
				break;
				case 1:
					if(x==s-1 || d[y][x+2]==1)f=2;
					else x++;
				break;
				case 2:
					if(y==s-1 || d[y+2][x]==1)f=3;
					else y++;
				break;
				case 3:
					if(x==0 || (x>=2 &&d[y][x-2]==1))f=0;
					else x--;
				break;
			}
			/*printf("%d %d %d\n",x,y,f);
			for(i=0;i<s;i++){
				for(j=0;j<s;j++){
					if(i!=y || j!=x)printf("%s",d[i][j]?"#":" ");
					else printf("A");
				}
				printf("\n");
			}*/
		}
		d[y][x]=1;
		printf("%s",(k==0)?"":"\n");
		for(i=0;i<s;i++){
			//printf("%d",i);
			for(j=0;j<s;j++)printf("%s",d[i][j]?"#":" ");
			printf("\n");
		}
	}
	return 0;
}