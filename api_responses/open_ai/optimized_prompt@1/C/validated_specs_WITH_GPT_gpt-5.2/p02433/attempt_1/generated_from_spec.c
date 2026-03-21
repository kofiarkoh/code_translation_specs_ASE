#include <stdio.h>
#include <stdlib.h>

typedef struct Node {
    int value;
    struct Node *prev;
    struct Node *next;
    int is_end_marker;
} Node;

static void die_move_error(void) {
    /* Specification only requires "an error message" and non-zero exit.
       Keep message minimal and deterministic. */
    fputs("Error\n", stderr);
    exit(1);
}

int main(void) {
    int ncmd;
    if (scanf("%d", &ncmd) != 1) {
        /* Undefined by spec for malformed input; return non-zero quietly. */
        return 1;
    }

    /* Sequence starts with a single distinguished internal end marker. */
    Node *end = (Node *)malloc(sizeof(Node));
    /* Allocation failure: undefined by spec; we do not handle. */
    end->prev = NULL;
    end->next = NULL;
    end->is_end_marker = 1;

    Node *front = end;   /* front element pointer (may be end marker) */
    Node *cursor = end;  /* cursor always points to some element */

    for (int i = 0; i < ncmd; i++) {
        char cmd;
        if (scanf(" %c", &cmd) != 1) {
            /* Undefined by spec for malformed input. */
            return 1;
        }

        if (cmd == 'I') {
            int v;
            if (scanf("%d", &v) != 1) return 1;

            Node *nn = (Node *)malloc(sizeof(Node));
            nn->value = v;
            nn->is_end_marker = 0;

            /* Insert immediately before cursor. */
            nn->next = cursor;
            nn->prev = cursor->prev;
            if (cursor->prev) {
                cursor->prev->next = nn;
            } else {
                /* cursor was at front */
                front = nn;
            }
            cursor->prev = nn;

            /* Cursor becomes the newly inserted element. */
            cursor = nn;

        } else if (cmd == 'M') {
            int offset;
            if (scanf("%d", &offset) != 1) return 1;

            /* Move stepwise; error if stepping outside bounds. */
            if (offset < 0) {
                while (offset < 0) {
                    if (cursor->prev == NULL) {
                        /* Attempt to move left at front element. */
                        die_move_error();
                    }
                    cursor = cursor->prev;
                    offset++;
                }
            } else {
                while (offset > 0) {
                    if (cursor->next == NULL) {
                        /* Attempt to move right when no next element. */
                        die_move_error();
                    }
                    cursor = cursor->next;
                    offset--;
                }
            }

        } else if (cmd == 'E') {
            /* Erase element at cursor if not end marker; else no effect. */
            if (!cursor->is_end_marker) {
                Node *to_free = cursor;
                Node *succ = cursor->next;

                if (to_free->prev) {
                    to_free->prev->next = to_free->next;
                } else {
                    /* removing front */
                    front = to_free->next;
                }
                if (to_free->next) {
                    to_free->next->prev = to_free->prev;
                }

                cursor = succ; /* cursor becomes next element (toward back) */
                free(to_free);
            }
        } else {
            /* Undefined by spec for malformed command; do nothing. */
        }
    }

    /* Output remaining sequence elements from front to back, excluding end marker. */
    for (Node *p = front; p != NULL; p = p->next) {
        if (p->is_end_marker) break;
        printf("%d\n", p->value);
    }

    /* Normal completion: reclaiming memory not specified; we can free safely. */
    /* Free all nodes including end marker. */
    Node *p = front;
    while (p) {
        Node *next = p->next;
        free(p);
        p = next;
    }

    return 0;
}
