#include <stdio.h>

int land[50][50]={0};

void dfs(int a, int b, int c, int d){

  int i,j;
  
  land[a][b]=0;

  for(i = -1; i <= 1; i++) for(j = -1; j <= 1; j++) if(a+i >= 0 && b+j >= 0 && a+i < c && b+j < d) if(land[a+i][b+j] == 1) dfs(a+i,b+j,c,d);

}

int main(){

  int w,h,result=0,i,j;

  while(1){
    
    scanf("%d %d",&w,&h);
    
    if(w == 0 && h == 0) return 0;
    
    result=0;
    
    for(i = 0; i < h; i++) for(j = 0; j < w; j++) scanf("%d",&land[j][i]);
    
    for(i = 0; i < h; i++){
      
      for(j = 0; j < w; j++){
	
	if(land[j][i] == 1){
	  
	  result++;
	  
	  dfs(j,i,w,h);
	  
	}
	
      }
      
    }
    
    printf("%d\n",result);
    
  }
  
}