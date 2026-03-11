#include<stdio.h>

int n;
int a[20][20];

void rot(int r, int c, int size, int angle){
  int time = angle/90, i;
  for(i=0;i<time;i++){
    int temp[20][20];
    int j, k;
    for(j=0;j<n;j++){
      for(k=0;k<n;k++) temp[j][k] = a[j][k];
    }
    int l=r-1;
    for(j=size-1;j>=0;j--){
      for(k=0;k<size;k++) a[r-1+k][c-1+j] = temp[l][c-1+k];
      l++;
    }
  }
  return;
}

void rev(int r, int c, int size){
  int i, j;
  for(i=r-1;i<r+size-1;i++){
    for(j=c-1;j<c+size-1;j++) a[i][j] = (a[i][j]+1)%2;
  }
  return;
}

void shf(int r, int o){
  int lr;
  if(o==2) lr=1;
  else lr=-1;

  int i, temp[n];
  for(i=0;i<n;i++) temp[i]=a[r-1][i];
  for(i=0;i<n;i++){
    a[r-1][i] = temp[(i+lr+n)%n];
  }
  return;
}


void irev(int r, int c, int is){
  int ser[4]={0, 1, 0, -1};
  int i;
  a[r][c] = is;
  for(i=0;i<4;i++){
    int nr = r+ser[i], nc = c+ser[i^1];
    if(a[nr][nc]!=is && nr>=0 && nc>=0 && nr<n && nc<n){
      irev(nr, nc, is);
    }
  }    
  return;
}

int main(){
  int m;
  scanf("%d %d", &n, &m);
  int i, j;
  for(i=0;i<n;i++){
    for(j=0;j<n;j++) scanf("%d", &a[i][j]);
  }

  int ope;
  for(ope=0;ope<m;ope++){
    int o;
    scanf("%d", &o);

    if(o==0){
      int r, c, size, angle;
      scanf("%d %d %d %d", &r, &c, &size, &angle);
      rot(r, c, size, angle);
    }
    else if(o==1){
      int r, c, size;
      scanf("%d %d %d", &r, &c, &size);
      rev(r, c, size);
    }
    else if(o==2 || o==3){
      int r;
      scanf("%d", &r);
      shf(r, o);
    }
    else if(o==4){
      int r, c;
      scanf("%d %d", &r, &c);
      irev(r-1, c-1, (a[r-1][c-1]+1)%2);
    }
  }
  
  for(i=0;i<n;i++){
    for(j=0;j<n;j++) printf("%d%c", a[i][j], " \n"[j==n-1]);
  }
  return 0;
}
  

