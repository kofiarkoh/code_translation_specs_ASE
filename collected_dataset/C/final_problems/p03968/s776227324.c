#include<stdio.h>

int main(){
  int N,i1,i2,i3,i4,i5,i6,j,k,l,m,n,c0,c1,c2,c3,c4,c5,c6,c7,count=0;
  scanf("%d",&N);
  int c[N][4];
  for(i1=0;i1<N;++i1) scanf("%d %d %d %d",&c[i1][0],&c[i1][1],&c[i1][2],&c[i1][3]);
  for(i1=0;i1<N-5;++i1){
    c0=c[i1][0];c1=c[i1][1];c2=c[i1][2];c3=c[i1][3];
    for(i2=i1+1;i2<N;++i2){
      for(j=0;j<4;++j){
        if(c0==c[i2][j]&&c1==c[i2][(j+3)%4]){
          c4=c[i2][(j+1)%4];
          c5=c[i2][(j+2)%4];
          for(i3=i1+1;i3<N;++i3){
            if(i3==i2) continue;
            for(k=0;k<4;++k){
              if(c1==c[i3][k]&&c5==c[i3][(k+1)%4]&&c2==c[i3][(k+3)%4]){
                c6=c[i3][(k+2)%4];
                for(i4=i1+1;i4<N;++i4){
                  if(i4==i2||i4==i3) continue;
                    for(l=0;l<4;++l){
                      if(c2==c[i4][l]&&c6==c[i4][(l+1)%4]&&c3==c[i4][(l+3)%4]){
                        c7=c[i4][(l+2)%4];
                        for(i5=i1+1;i5<N;++i5){
                          if(i5==i2||i5==i3||i5==i4) continue;
                            for(m=0;m<4;++m){
                              if(c4==c[i5][m]&&c0==c[i5][(m+1)%4]&&c7==c[i5][(m+3)%4]&&c3==c[i5][(m+2)%4]){
                                for(i6=i1+1;i6<N;++i6){
                                  if(i6==i2||i6==i3||i6==i4||i6==i5) continue;
                                  for(n=0;n<4;++n){
                                    if(c5==c[i6][n]&&c4==c[i6][(n+1)%4]&&c6==c[i6][(n+3)%4]&&c7==c[i6][(n+2)%4]){
                                      ++count;
                                    }
                                  }
                                }
                              }
                            }
                        }
                      }
                    }
                }
              }
            }
          }
        }
      }
    }
  }

  printf("%d\n",count);

  return 0;
}