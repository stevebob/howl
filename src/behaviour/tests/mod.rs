use behaviour::*;

type TestGraph = Graph<isize, &'static str>;
type TestLeafFn = LeafFn<isize, &'static str>;
type TestCheckFn = CheckFn<isize>;

fn action(s: &'static str) -> TestLeafFn {
    Box::new(move |_| LeafResolution::Yield(s))
}

fn create_a() -> (TestGraph, NodeIndex) {
    let mut graph = Graph::new();

    let hello = graph.add_leaf(action("hello"));
    let world = graph.add_leaf(action("world"));

    let all = graph.add_collection(CollectionNode::All(vec![hello, world]));

    let forever = graph.add_collection(CollectionNode::Forever(all));

    (graph, forever)
}

fn create_b() -> (TestGraph, NodeIndex, NodeIndex) {
    let (mut graph, a_root) = create_a();

    let one = graph.add_leaf(action("one"));
    let two = graph.add_leaf(action("two"));
    let three = graph.add_leaf(action("three"));

    let root = graph.add_collection(CollectionNode::All(vec![one, two, three]));

    (graph, a_root, root)
}

fn create_c() -> (TestGraph, NodeIndex) {
    let (mut graph, a_root, b_root) = create_b();

    let rti = graph.add_leaf(Box::new(|_| LeafResolution::ReturnFromInterrupt));

    let handler = graph.add_collection(CollectionNode::All(vec![b_root, rti]));

    let root = graph.add_check(a_root, Box::new(move |k| {
        if k == 0 {
            None
        } else {
            Some(CheckResolution::Interrupt(handler))
        }
    }));

    (graph, root)
}

fn create_d() -> (TestGraph, NodeIndex) {
    let (mut graph, _, b_root) = create_b();

    let root = graph.add_check(b_root, Box::new(move |k| {
        if k == 0 {
            None
        } else {
            Some(CheckResolution::Restart)
        }
    }));

    (graph, root)
}

#[test]
fn forever() {
    let (graph, root) = create_a();

    let mut state = State::new();
    state.initialise(&graph, root).unwrap();

    assert_eq!(state.run_to_action(&graph, 0).unwrap(), "hello");
    state.report_action_result(true).unwrap();
    assert_eq!(state.run_to_action(&graph, 0).unwrap(), "world");
    state.report_action_result(true).unwrap();
    assert_eq!(state.run_to_action(&graph, 0).unwrap(), "hello");
    state.report_action_result(true).unwrap();
}

#[test]
fn interrupt() {
    let (graph, root) = create_c();

    let mut state = State::new();
    state.initialise(&graph, root).unwrap();

    // test normal operation
    assert_eq!(state.run_to_action(&graph, 0).unwrap(), "hello");
    state.report_action_result(true).unwrap();
    assert_eq!(state.run_to_action(&graph, 0).unwrap(), "world");
    state.report_action_result(true).unwrap();
    assert_eq!(state.run_to_action(&graph, 0).unwrap(), "hello");
    state.report_action_result(true).unwrap();

    // trigger interrupt
    assert_eq!(state.run_to_action(&graph, 1).unwrap(), "one");
    state.report_action_result(true).unwrap();
    assert_eq!(state.run_to_action(&graph, 0).unwrap(), "two");
    state.report_action_result(true).unwrap();
    assert_eq!(state.run_to_action(&graph, 0).unwrap(), "three");
    state.report_action_result(true).unwrap();

    // should return from interrupt here
    assert_eq!(state.run_to_action(&graph, 0).unwrap(), "world");
    state.report_action_result(true).unwrap();
}

#[test]
fn restart() {
    let (graph, root) = create_d();

    let mut state = State::new();
    state.initialise(&graph, root).unwrap();

    // normal operation
    assert_eq!(state.run_to_action(&graph, 0).unwrap(), "one");
    state.report_action_result(true).unwrap();
    assert_eq!(state.run_to_action(&graph, 0).unwrap(), "two");
    state.report_action_result(true).unwrap();

    // trigger restart
    assert_eq!(state.run_to_action(&graph, 1).unwrap(), "one");
    state.report_action_result(true).unwrap();
    assert_eq!(state.run_to_action(&graph, 0).unwrap(), "two");
    state.report_action_result(true).unwrap();
    assert_eq!(state.run_to_action(&graph, 0).unwrap(), "three");
    state.report_action_result(true).unwrap();
}
