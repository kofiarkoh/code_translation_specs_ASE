#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define L 50
#define GET(x,y) *(map+((y-1)*(n+1))+(x-1))
#define GETANS(x,y,xx,yy) GETANS2((x-1),(y-1),(xx),(yy))
#define GETANS2(x,y,xx,yy) *(ansmap  + (yy*n*n*L)+(xx*n*n)+(y*n)+x  )
#define GETANS2(x,y,xx,yy) *(ansmap  + (yy*n*n*L)+(xx*n*n)+(y*n)+x  )
#define NUM(x,y) ((GET(x,y))-'0')

int n;
char* map;
char* ansmap;

int setHash(int Xx,int Xy,int Yx,int Yy, int ans)
{
  if( L <= (Yx-Xx) ) return 0;
  if( L <= (Yy-Xy) ) return 0;
  // 0:unknown 1:ok -1:ng
  GETANS(Xx,Xy,Yx-Xx,Yy-Xy) = ans;
  return 0;
}
  
int getHash(int Xx,int Xy,int Yx,int Yy)
{
  if( L <= (Yx-Xx) ) return 0;
  if( L <= (Yy-Xy) ) return 0;
//  fprintf(stderr,"%d,%d,%d,%d/",Xx,Xy,Yx-Xx,Yy-Xy);
  return GETANS(Xx,Xy,Yx-Xx,Yy-Xy);
}

int isOK(int Xx,int Xy,int Yx,int Yy)
{
  int ans;
  if( (Xx==Yx)&&(Xy==Yy) ) return 1;
  if(Xx>Yx) return 0; 
  if(Xy>Yy) return 0; 
  ans = getHash(Xx,Xy,Yx,Yy);
  if ( ans != 0 )
  {
    if(ans>0) return 1;
    return 0;
  }
  if( '#' == GET(Xx, Xy) ){ 
    setHash(Xx,Xy,Yx,Yy,-1);
    return 0;
  }
  if( isOK(Xx+1, Xy, Yx, Yy) ) {
    setHash(Xx,Xy,Yx,Yy,1);
    return 1;
  }
  if( isOK(Xx, Xy+1, Yx, Yy) ) {
    setHash(Xx,Xy,Yx,Yy,1);
    return 1;
  }
  setHash(Xx,Xy,Yx,Yy,-1);
  return 0;
}

int calc(int Xx,int Xy,int Yx,int Yy)
{
  int ok=isOK(Xx,Xy,Yx,Yy);
  if(0==ok) return 0;
  return ok*NUM(Xx,Xy)*NUM(Yx,Yy);
}

int main(void)
{
  int i;
  int sum=0;
  int Xx,Xy,Yx,Yy;
  scanf("%d",&n); 
//  printf ("%d\n",n);
  map = malloc((n+1)*(n+1));
  ansmap = calloc(n*n*50*50,1);

  for(i=0;i<n;i++)
    scanf("%s",(map+(n+1)*i)); 
//  for(i=0;i<n;i++)
//    printf("%s\n",(map+(n+1)*i));

  for(Xx=1;Xx<=n;Xx++)
  {
    for(Xy=1;Xy<=n;Xy++)
    {
      if( '#' == GET(Xx, Xy) ) continue;
      for(Yx=Xx;Yx<=n;Yx++)
      {
        for(Yy=Xy;Yy<=n;Yy++)
        {
          if( (Xx>=Yx) && (Xy>=Yy) ) continue;
          if( '#' == GET(Yx, Yy) )
          {
            if(Xx==Yx)break; 
            continue;
          }
          sum += calc(Xx,Xy,Yx,Yy);
        }
      }      
    }
  }
  printf( "%d\n", sum );
//  fprintf(stderr,"clock = %u\n",clock());
  return 0;
}