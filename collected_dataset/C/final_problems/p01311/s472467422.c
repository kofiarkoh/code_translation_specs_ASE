#include <stdio.h>
#include <string.h>

int nat_m,nat_h[111111],nat_n[111111*2],nat_t[111111*2];
int nat_c[111111];
int neko_m,neko_h[111111],neko_n[111111*2],neko_t[111111*2];
int neko_c[111111],neko_c_natsu[111111];
int q[555555],inq[111111],hd,tl;
int nat_natsu_c[111111],neko_natsu_c[111111];

int main(void)
{
  int n,m,nat,neko,u,v,i,e,p,res,T;
  char c;
  scanf("%d",&T);
  while( T-- ) {
    scanf("%d%d",&n,&m);
    scanf("%d%d",&nat,&neko);
    nat_m = neko_m = 0;
    for(i = 0; i <= n; i++) {
      nat_h[i] = neko_h[i] = -1;
      neko_c[i] = neko_c_natsu[i] = 1<<21;
      nat_c[i] = 1<<21;
    }
#define add_edge1(n,h,t,m,u,v) (n[m]=h[u],h[u]=m,t[m]=v,++m)
    for( i = 0; i < m; i++ ) {
      scanf("%d%d %c",&u,&v,&c);
      if( c == 'N' ) {
        add_edge1(nat_n,nat_h,nat_t,nat_m,u,v);
        add_edge1(nat_n,nat_h,nat_t,nat_m,v,u);
      }
      if( c == 'L' ) {
        add_edge1(neko_n,neko_h,neko_t,neko_m,u,v);
        add_edge1(neko_n,neko_h,neko_t,neko_m,v,u);
      }
    }
    nat_c[nat] = 0;
    hd = tl = 0;
    q[tl++] = nat;
    while( hd != tl ) {
      p = q[hd++];
      for( e = nat_h[p]; e != -1; e = nat_n[e] ) {
        if( nat_c[nat_t[e]] > nat_c[p]+1 ) {
          nat_c[nat_t[e]] = nat_c[p]+1;
          q[tl++] = nat_t[e];
        }
      }
    }
    memset(inq,0,sizeof(inq));
    hd = tl = 0;
    q[tl++] = 0; neko_c_natsu[0] = 0; inq[neko] = 1;
    while( hd != tl ) {
      p = q[hd++]; inq[p] = 0;
      if( neko_c_natsu[p] == 0 ) {
        for(e = neko_h[p]; e != -1; e = neko_n[e]) {
          if( neko_c_natsu[neko_t[e]] > neko_c_natsu[p] ) {
            neko_c_natsu[neko_t[e]] = neko_c_natsu[p];
            if( !inq[neko_t[e]] ) {
              inq[neko_t[e]] = 1;
              q[tl++] = neko_t[e];
            }
          }
        }
      }
      for(e = nat_h[p]; e != -1; e = nat_n[e]) {
        if( neko_c_natsu[nat_t[e]] > neko_c_natsu[p]+1 ) {
          neko_c_natsu[nat_t[e]] = neko_c_natsu[p]+1;
          if( !inq[nat_t[e]] ) {
            inq[nat_t[e]] = 1;
            q[tl++] = nat_t[e];
          }
        }
      }
    }
    memset(inq,0,sizeof(inq));
    hd = tl = 0;
    q[tl++] = neko; neko_c[neko] = 0; inq[neko] = 1;
    while( hd != tl ) {
      p = q[hd++]; inq[p] = 0;
      if( neko_c[p] == 0 ) {
        for(e = neko_h[p]; e != -1; e = neko_n[e]) {
          if( neko_c[neko_t[e]] > neko_c[p] ) {
            neko_c[neko_t[e]] = neko_c[p];
            if( !inq[neko_t[e]] ) {
              inq[neko_t[e]] = 1;
              q[tl++] = neko_t[e];
            }
          }
        }
      }
      for(e = nat_h[p]; e != -1; e = nat_n[e]) {
        if( neko_c[nat_t[e]] > neko_c[p]+1 ) {
          neko_c[nat_t[e]] = neko_c[p]+1;
          if( !inq[nat_t[e]] ) {
            inq[nat_t[e]] = 1;
            q[tl++] = nat_t[e];
          }
        }
      }
    }
    res = 1<<21;
    if( neko_c[0] == 0 ) res = 0;
    for( i = 0; i <= n; i++ ) {
      int k = nat_c[i]+neko_c[i]+neko_c_natsu[i];
      if( res > k ) res = k;
    }
    printf("%d\n",res);
  }
  return 0;
}