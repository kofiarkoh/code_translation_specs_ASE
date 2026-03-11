#include <stdio.h>
#include <math.h>

#define sq(x) ((x) * (x))
int color[8][3] ={ {0,0,0},
		   {0,0,255},
		   {0,255,0},
		   {0,255,255},
		   {255,0,0},
		   {255,0,255},
		   {255,255,0},
		   {255,255,255} };

int convert(char a, char b){
  int x, y;
  if(a >= 'a' && a <= 'f') x = (a - 'a' + 10) * 16;
  else x = (a - '0') * 16;
  if(b >= 'a' && a <= 'f') y = b - 'a' + 10;
  else y = b - '0';
  return x + y;
}

int main(){
  int n, i, j, c[3], dmin, dk;
  char rgb[16], s;

  while(scanf("%c", &s) && s != '0'){
    scanf("%s", rgb);
    getchar();
    c[0] = c[1] = c[2] = 0;
    for(i = 0, j = 0; i < 6; i += 2, j++)
      c[j] = convert(rgb[i], rgb[i + 1]);
    
    dmin = 9999999;
    int res;
    for(i = 0; i < 8; ++i){
      dk = sqrt(sq(c[0] - color[i][0]) + sq(c[1] - color[i][1]) + sq(c[2] - color[i][2]));
      if(dmin > dk){
	dmin = dk;
	res = i;
      }
    }

    switch(res){
    case 0: puts("black"); break;
    case 1: puts("blue"); break;
    case 2: puts("lime"); break;
    case 3: puts("aqua"); break;
    case 4: puts("red"); break;
    case 5: puts("fuchsia"); break;
    case 6: puts("yellow"); break;
    case 7: puts("white"); break;
    }
  }
  return 0;
}