/*
 * x.c - x
 *
 * Date   : 2020/12/07
 */

extern void call_from_c();

int main(void)
{
    call_from_c();
    return 0;
}

/* Local Variables: */
/* compile-command: "gcc xc.c -L../../../target/debug -ladvance -o xc" */
/* End: */
