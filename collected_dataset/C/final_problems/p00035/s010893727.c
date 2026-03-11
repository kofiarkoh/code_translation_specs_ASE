#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define BUFFER_MAX 1024
#define APEXES_NUM 4

typedef struct {
     double x;
     double y;
} Point;

int FuncIsPointLorR(const Point *point, const Point *bp, const Point *ep)
{
     Point line[2];
     double sample;

     line[0].x = ep->x - bp->x;
     line[0].y = ep->y - bp->y;

     line[1].x = point->x - bp->x;
     line[1].y = point->y - bp->y;

     sample = (line[0].x * line[1].y) - (line[0].y * line[1].x);

     if (sample == 0) {
          return 0;
     } else if (sample > 0) {
          return 1;
     } else if (sample < 0) {
          return -1;
     } else {
          exit(EXIT_FAILURE);
     }
}

int main(void)
{
     char  buf[BUFFER_MAX];
     Point apexes[APEXES_NUM];

     while (fgets(buf, 1023, stdin) != NULL) {
          sscanf(buf, "%lf,%lf,%lf,%lf,%lf,%lf,%lf,%lf",
                 &(apexes[0].x), &(apexes[0].y),
                 &(apexes[1].x), &(apexes[1].y),
                 &(apexes[2].x), &(apexes[2].y),
                 &(apexes[3].x), &(apexes[3].y));

          if ( (FuncIsPointLorR(&apexes[1], &apexes[0], &apexes[2]) < 0 &&
                       FuncIsPointLorR(&apexes[2], &apexes[1], &apexes[3]) < 0 &&
                       FuncIsPointLorR(&apexes[3], &apexes[2], &apexes[0]) < 0 &&
                       FuncIsPointLorR(&apexes[0], &apexes[3], &apexes[1]) )   ||
                      (FuncIsPointLorR(&apexes[1], &apexes[0], &apexes[2]) > 0 &&
                       FuncIsPointLorR(&apexes[2], &apexes[1], &apexes[3]) > 0 &&
                       FuncIsPointLorR(&apexes[3], &apexes[2], &apexes[0]) > 0 &&
                       FuncIsPointLorR(&apexes[0], &apexes[3], &apexes[1]) )     ) {
               puts("YES");
          } else {
               puts("NO");
          }
     }
     
     return 0;
}