#include <stdio.h>
#include <stdlib.h>
int P[3][3],tmp[3][3];
int Dx[4] = {0,1,0,-1};
int Dy[4] = {1,0,-1,0};
void emp(int *x,int *y) {
  int i,j;
  for(i=0; i<3; i++) {
    for(j=0; j<3; j++) {
      if(!P[i][j]) {
	*x = j;
	*y = i;
      }
    }
  }
}
void swap(int *s,int *v){
  int tmp;
  tmp = *s;
  *s = *v;
  *v = tmp;
}   
int calc(){
  int i,t,sum=0; 
  for(i=0; i<9; i++){
    t = P[i/3][i%3];
    if(t){
      t--;
      sum += abs(i/3 - t/3) + abs(i%3 - t%3);
    }
  }    
  return sum;
}
int dfs(int x,int y,int p,int d,const int lim){
  int i,hi,x2,y2,res;   
  hi = calc();
  if(hi==0) {
    return d;
  }
  if(d+hi>lim) {
    return -1;
  }
  for(i=0; i<4; i++){
    x2 = x+Dx[i];
    y2 = y+Dy[i];      
    if(~p && i==(p+2)%4) continue;
    if(!(0<=x2 && x2<3 && 0<=y2 && y2<3)) continue;    
    swap(&P[y][x],&P[y2][x2]);
    res = dfs(x2,y2,i,d+1,lim);
    if(~res) {
      return res;
    }
    swap(&P[y][x],&P[y2][x2]);
  }   
  return -1;
}    
int main() {
  int i,j,l,ans,x,y;
        
  for(i=0; i<3; i++){
    for(j=0; j<3; j++) scanf("%d",&P[i][j]);
  }
         
  emp(&x,&y);
  for(l=0; l<100; l++){
    for(i=0; i<3; i++){
      for(j=0; j<3; j++) tmp[i][j] = P[i][j];
    }
            
    ans = dfs(x,y,-1,0,l);
    if(ans!=-1){
      printf("%d\n",ans);
      break;
    }
            
    for(i=0; i<3; i++){
      for(j=0; j<3; j++) P[i][j] = tmp[i][j];
    }
            
    ans = -1;
    if(l==99) printf("%d\n",ans);
  }
        
  return 0;
}

