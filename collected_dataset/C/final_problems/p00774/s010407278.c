#include<stdio.h>
int main(){
  int h,i,j,c,s,f,k;
  while(scanf("%d",&h),h){
    int d[20][6]={0};
    for(i=s=0;i<h;i++){
      for(j=0;j<5;j++)scanf("%d",&d[h-i-1][j]);
    }
    /*for(i=0;i<20;i++){
      for(j=0;j<5;j++){
	printf("%d ",d[i][j]);
      }
      printf("\n");
      }printf("\n");//*/
    f=1;
    while(f){
      for(i=f=0;i<h;i++){
	c=1;
	for(j=0;j<5;j++){
	  if(d[i][j]==d[i][j+1]&&d[i][j])c++;
	  else{
	    if(c>2){
	      f=1;
	      for(k=0;k<c;k++){
		s+=d[i][j-k];
		d[i][j-k]=0;
	      }
	    }
	    c=1;
	  }//printf("%d ",c);
	}//printf("\n");
      }//printf("\n");
      for(i=0;i<5;i++){
	for(j=c=0;j<10;j++){
	  d[j-c][i]=d[j][i];
	  if(d[j][i]==0&&j<h)c++;
	}
      }
      /*for(i=0;i<20;i++){
	for(j=0;j<5;j++)printf("%d ",d[i][j]);
	printf("\n");
	}printf("\n");//*/
    }
    printf("%d\n",s);
  }
  return 0;
}

  