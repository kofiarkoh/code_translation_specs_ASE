#include<stdio.h>
int main(){
  int h,w,i,j,gx,gy,t,r,x[400],y[400],g[10];
  int X[]={ 0, 0,-1, 1, 0};
  int Y[]={ 0, 1, 0, 0,-1};
  char m[20][20],p;
  while(scanf("%d %d",&h,&w),w||h){
    int c[20][20]={0};
    for(i=0;i<h;i++){
      for(j=0;j<w;j++){
	scanf(" %c ",&m[i][j]);//printf("%c",m[i][j]);
	if(m[i][j]=='A'){
	  y[0]=i;
	  x[0]=j;
	  t=0;
	  c[i][j]=r=1;
	}
	if(m[i][j]=='B'){
	  gy=i;
	  gx=j;
	}
      }//printf("\n");
    }
    for(t=0;r-t;t++){
      for(i=1;i<5;i++){
	if(0<=x[t]+X[i]&&x[t]+X[i]<w&&
	   0<=y[t]+Y[i]&&y[t]+Y[i]<h){
	  if(m[y[t]+Y[i]][x[t]+X[i]]!='#'&&
	     c[y[t]+Y[i]][x[t]+X[i]]==0){
	    c[y[t]+Y[i]][x[t]+X[i]]=c[y[t]][x[t]]+1;
	    x[r]=x[t]+X[i];
	    y[r]=y[t]+Y[i];
	    r++;
	  }
	}
      }
    }
    /*for(i=0;i<h;i++){
      for(j=0;j<w;j++){
	printf("%4d",c[i][j]);
      }
      printf("\n");
      }*/
    for(i=0;scanf("%c",&p),p-10;i++){//printf("%c",p);
      if(p=='5')g[i]=0;
      else	g[i]=(p-'0')/2;
    }//printf("\n");
    for(j=0;1;j++){
      gy+=Y[g[j%i]];
      gx+=X[g[j%i]];
      if(gx<0 )gx=0;
      if(w==gx)gx--;
      if(gy<0 )gy=0;
      if(h==gy)gy--;
      if(c[gy][gx]<=j+2&&c[gy][gx])break; 
      if(j==400)break;
    }
    if(j-400)printf("%d %d %d\n",j+1,gy,gx);
    else     printf("impossible\n");
  }
  return 0;
}