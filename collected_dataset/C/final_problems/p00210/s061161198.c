#include<stdio.h>
int main(){
  int X[]={ 0,-1,0,1, 0,-1,0};
  int Y[]={-1, 0,1,0,-1, 0,1};
  int d[900][4];//x,y,d,c
  char m[30][31];
  int w,h,i,j,k,l,c,xc;
  int x[900],y[900];
  while(1){
    scanf("%d %d",&w,&h);
    if(!(w||h))break;
    scanf("\n");
    c=0;
    k=0;
    xc=0;
    for(i=0;i<h;i++){
      for(j=0;j<=w;j++){
	scanf("%c",&m[i][j]);
	d[k][3]=1;
	if(m[i][j]=='E'){
	  d[k][0]=j;
	  d[k][1]=i;
	  d[k][2]=2;
	  k++;
	}
	if(m[i][j]=='N'){
	  d[k][0]=j;
	  d[k][1]=i;
	  d[k][2]=3;
	  k++;
	}
	if(m[i][j]=='W'){
	  d[k][0]=j;
	  d[k][1]=i;
	  d[k][2]=0;
	  k++;
	}
	if(m[i][j]=='S'){
	  d[k][0]=j;
	  d[k][1]=i;
	  d[k][2]=1;
	  k++;
	}
	if(m[i][j]=='X'){
	  x[xc]=j;
	  y[xc]=i;
	  xc++;
	}
      }
    }
    /*for(i=0;i<k;i++){
      printf("d%d c%d\n",d[i][2],d[i][3]);
      }*/
    for(i=1;i<=180;i++){
      for(j=0;j<k;j++){//mukichange
	if(d[j][3]==0)continue;
	for(l=0;l<4;l++){
	  if(m[d[j][1]+Y[d[j][2]+l]][d[j][0]+X[d[j][2]+l]]=='.'||
	     m[d[j][1]+Y[d[j][2]+l]][d[j][0]+X[d[j][2]+l]]=='X'){
	    d[j][2]+=l-1;//printf("d%d\n",d[j][2]);
	    //d[j][3]=1;
	    if(d[j][2]<0)d[j][2]+=4;
	    if(d[j][2]>3)d[j][2]-=4;
	    break;
	  }
	}
      }//printf("%d\n",d[0][2]);
      for(j=0;j<4;j++){//junbannni idou
	for(l=0;l<k;l++){//printf("%d",d[l][2]);
	  if(d[l][3]==0)continue;
	  if(d[l][2]==j){//printf("%d\n",j);
	    if(m[d[l][1]+Y[1+j]][d[l][0]+X[1+j]]=='.'||
	       m[d[l][1]+Y[1+j]][d[l][0]+X[1+j]]=='X'){
	      if(m[d[l][1]+Y[1+j]][d[l][0]+X[1+j]]=='X')d[l][3]=0;
	      m[d[l][1]][d[l][0]]='.';
	      m[d[l][1]+Y[1+j]][d[l][0]+X[1+j]]='P';
	      d[l][1]+=Y[1+j];
	      d[l][0]+=X[1+j];
	    }
	    /*else if(m[d[l][1]+Y[1+j]][d[l][0]+X[1+j]]=='X'){
	      m[d[l][1]][d[l][0]]='.';
	      c++;
	      d[l][3]=0;
	      }*/
	  }
	}
      }
      for(j=0;j<xc;j++){
	if(m[y[j]][x[j]]=='P'){
	  m[y[j]][x[j]]='X';
	  c++;
	}
      }
      /*for(l=0;l<h;l++){
	for(j=0;j<w;j++){
	  printf("%c",m[l][j]);
	}
	printf("\n");
      }
      printf("%d\n",c);*/
      if(k==c)break;
    }
    if(k==0)i=0;
    if(i==181)printf("NA\n");
    else      printf("%d\n",i);
  }
  return 0;
}