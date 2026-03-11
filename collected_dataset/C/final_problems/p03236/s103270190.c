#include <stdio.h>
#include <stdlib.h>

#define GET(x,y) *(map+((y-1)*(n+1))+(x-1))
#define NUM(x,y) ((GET(x,y))-'0')

int n;
char* map;

int isOK(int Xx,int Xy,int Yx,int Yy)
{
  if( (Xx==Yx)&&(Xy==Yy) ) return 1;
  if(Xx>Yx) return 0; 
  if(Xy>Yy) return 0; 
  if( '#' == GET(Xx, Xy) ) return 0;
  if( isOK(Xx+1, Xy, Yx, Yy) ) return 1;
  if( isOK(Xx, Xy+1, Yx, Yy) ) return 1;
  return 0;
}

int calc(int Xx,int Xy,int Yx,int Yy)
{
  int x,y;
  int ok=isOK(Xx,Xy,Yx,Yy);
//  printf("%d,%d,%d,%d=%d/", Xx,Xy,Yx,Yy, ok);
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

  for(i=0;i<n;i++)
    scanf("%s",(map+(n+1)*i)); 
//  for(i=0;i<n;i++)
//    printf("%s\n",(map+(n+1)*i));

  for(Xx=1;Xx<=n;Xx++)
  {
    for(Xy=1;Xy<=n;Xy++)
    {
      if( '#' == GET(Xx, Xy) ) continue;
      for(Yx=1;Yx<=n;Yx++)
      {
        for(Yy=1;Yy<=n;Yy++)
        {
          if( (Xx>=Yx) && (Xy>=Yy) ) continue;
          if( '#' == GET(Yx, Yy) )
          {
//            if( (Xx == Yx) || (Xy == Yy ) ) break; 
            continue;
          }
          sum += calc(Xx,Xy,Yx,Yy);
        }
      }      
    }
  }
  printf( "%d", sum );
  return 0;
}