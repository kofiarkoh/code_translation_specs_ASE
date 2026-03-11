#include <stdio.h>
#include <stdlib.h>

/** ビッグボーナスの回数 */
static int b;
/** レギュラーボーナスの回数 */
static int r;
/** 通常ゲーム中でのブドウの回数 */
static int g;
/** チェリーの回数 */
static int c;
/** スターの回数 */
static int s;
/** 総ゲーム数 */
static int t;

int
main (
  int   argc,
  char *argv[ ]
  )
{
  for ( ; ; )
  {
    scanf ( "%d%d%d%d%d%d", &b, &r, &g, &c, &s, &t );
    if ( !( b | r | g | c | s | t ) ) break ;

    printf ( "%d\n"
      , 100                     /*< 元金 */
      - t * 3                   /*< スロットコスト */
      + ( b * 5 + r * 3 ) * 16  /*< ボーナス利益+スロットコスト差分 */
      + b * 15                  /*< 777 利益 */
      + r * 15                  /*< BAR 利益 */
      + g * 7                   /*< グレープ利益 */
      + c * 2                   /*< チェリー利益 */
      + s * 3 );                /*< スタースロットコスト差分 */
  }

  return ( EXIT_SUCCESS );
}