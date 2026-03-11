#include <stdio.h>
#include <math.h>
#include <stdlib.h>
 
int size[12][3],pie[12][500][2],sel[12][2000][4],nsel[12],order[12],list[12],q[500],mk[25][25];
int side[12][4][2],put[12][4][25][25];
int H,W,N,PL,SPACE,SET;
char map[25][25],piece[25][25];


void swap (int* x,int* y){
	int z;
	z=*x;
	*x=*y;
	*y=z;
}

void cover(int n,int x,int y,int dir,int val){
	int i,px,py,ok=1;
	int hei=size[n][0],wid=size[n][1],num=size[n][2];
	for(i=0;i<num;i++){
		px=pie[n][i][0]; py=pie[n][i][1];
		if(dir==0)mk[x+px][y+py]=val;
		if(dir==1)mk[x+py][y+hei-px-1]=val;
		if(dir==2)mk[x+hei-px-1][y+wid-py-1]=val;
		if(dir==3)mk[x+wid-py-1][y+px]=val;
	}
}

void paint(int n,int x,int y,int dir,char cc){
	int i,px,py,ok=1;
	int hei=size[n][0],wid=size[n][1],num=size[n][2];
	for(i=0;i<num;i++){
		px=pie[n][i][0]; py=pie[n][i][1];
		if(dir==0)map[x+px][y+py]=cc;
		if(dir==1)map[x+py][y+hei-px-1]=cc;
		if(dir==2)map[x+hei-px-1][y+wid-py-1]=cc;
		if(dir==3)map[x+wid-py-1][y+px]=cc;
	}
}

int isfit(int n,int x,int y,int dir){
	int i,px,py,ok=1;
	int hei=size[n][0],wid=size[n][1],num=size[n][2];
	for(i=0;i<num;i++){
		px=pie[n][i][0]; py=pie[n][i][1];
		if(dir==0 && map[x+px][y+py]=='#'){ok=0;break;}
		if(dir==1 && map[x+py][y+hei-px-1]=='#'){ok=0;break;}
		if(dir==2 && map[x+hei-px-1][y+wid-py-1]=='#'){ok=0;break;}
		if(dir==3 && map[x+wid-py-1][y+px]=='#'){ok=0;break;}
	}
	return ok;
}

void check(int n,int x,int y,int dir,int* ss){
	if(isfit(n,x,y,dir)){
		sel[n][*ss][0]=x;
		sel[n][*ss][1]=y;
		sel[n][*ss][2]=dir;
		put[n][dir][x+side[n][dir][0]][y+side[n][dir][1]]=SET;
		(*ss)++;
	}
}

int search(int line,int num,int left){
	int i,j,k,n,x,y,dir,ok;

	for(i=line;i<H;i++){
		for(j=0;j<W;j++){
			if(map[i][j]=='.')goto next;
		}
	}

	next:
	x=i;y=j;ok=0;
	
	for(i=0;i<num;i++){
		if(q[i]>=0){
			n=q[i];
			for(dir=0;dir<4;dir++){
				if(put[n][dir][x][y] && isfit(n,x-side[n][dir][0],y-side[n][dir][1],dir)){
					if(left==1)return 1;
					q[i]=-1;
					paint(n,x-side[n][dir][0],y-side[n][dir][1],dir,'#');
					ok=search(x,num,left-1);
					paint(n,x-side[n][dir][0],y-side[n][dir][1],dir,'.');
					q[i]=n;
				}
				if(ok)return 1;
			}
		}
	}

	return 0;
}

int main(){
    int i,j,k,hh,ww,num,q1,q2;
 
    while(scanf("%d %d",&H,&W)*(H+W)){
		SET++;

		for(i=0;i<H;i++)scanf("%s",map[i]);

		SPACE=0;
		for(i=0;i<H;i++){
			for(j=0;j<W;j++){
				if(map[i][j]=='.')SPACE++;
			}
		}

		scanf("%d",&N);
		for(i=0;i<N;i++){
			order[i]=i;
			scanf("%d %d",&hh,&ww);
			size[i][0]=hh;size[i][1]=ww;
			for(j=0;j<hh;j++)scanf("%s",piece[j]);

			q1=1;q2=1;num=0;
			if(hh!=ww)q1=0;
			for(j=0;j<hh;j++){
				for(k=0;k<ww;k++){
					if(piece[j][k]!=piece[k][hh-1-j])q1=0;
					if(piece[j][k]!=piece[hh-1-j][ww-1-k])q2=0;
					if(piece[j][k]=='#'){
						pie[i][num][0]=j;
						pie[i][num][1]=k;
						num++;
					}
				}
			}
			size[i][2]=num;

			for(j=0;j<hh;j++){for(k=0;k<ww;k++){if(piece[j][k]=='#'){side[i][0][0]=j;side[i][0][1]=k;goto out0;}}}
			out0:
			for(j=0;j<ww;j++){for(k=0;k<hh;k++){if(piece[hh-1-k][j]=='#'){side[i][1][0]=j;side[i][1][1]=k;goto out1;}}}
			out1:
			for(j=0;j<hh;j++){for(k=0;k<ww;k++){if(piece[hh-1-j][ww-1-k]=='#'){side[i][2][0]=j;side[i][2][1]=k;goto out2;}}}
			out2:
			for(j=0;j<ww;j++){for(k=0;k<hh;k++){if(piece[k][ww-1-j]=='#'){side[i][3][0]=j;side[i][3][1]=k;goto out3;}}}
			out3:

			nsel[i]=0;
			for(j=0;j<H-hh+1;j++){
				for(k=0;k<W-ww+1;k++){
					check(i,j,k,0,&nsel[i]);
					if(!q2)check(i,j,k,2,&nsel[i]);
				}
			}

			if(!q1){
				for(j=0;j<H-ww+1;j++){
					for(k=0;k<W-hh+1;k++){
						check(i,j,k,1,&nsel[i]);
						if(!q2)check(i,j,k,3,&nsel[i]);
					}
				}
			}

		}

		for(i=1;i<N;i++){
			for(j=i;j>0;j--){
				if(nsel[order[j-1]]>nsel[order[j]])swap(&order[j-1],&order[j]);
				else break;
			}
		}

		for(scanf("%d",&PL);PL>0;PL--){
			scanf("%d",&num);
			for(i=0;i<N;i++)list[i]=0;
			
			for(i=0;i<H;i++){
				for(j=0;j<W;j++){
					mk[i][j]=-1;
				}
			}

			for(i=0,j=0;i<num;i++){
				scanf("%d",&k);
				list[k-1]++;
				j+=size[k-1][2];
			}

			if(SPACE!=j){printf("NO\n");continue;}

			for(i=0,k=0;i<N && k<num;i++){
				for(j=list[order[i]];j>0;j--){
					q[k]=order[i];
					k++;
				}
			}
			/*
			for(i=0;i<num;i++){
				for(j=nsel[q[i]]-1;j>=0;j--){
					cover(q[i],sel[q[i]][j][0],sel[q[i]][j][1],sel[q[i]][j][2],0);
				}
			}

			q1=1;
			for(j=0;j<H;j++){
				for(k=0;k<W;k++){
					if(mk[j][k]==-1 && map[j][k]=='.'){q1=0;goto out;}
				}
			}

			out:if(!q1){printf("NO\n");continue;}
			*/
			if(search(0,num,num))printf("YES\n"); else printf("NO\n");
			
		}

    }
  
    return 0;
}